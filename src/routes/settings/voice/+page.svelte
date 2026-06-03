<script lang="ts">
import { Badge, Button, Field, Input, Select, Setting, Switch } from '@waifu/ui'
import { onMount } from 'svelte'
import {
	configToVoice,
	defaultSTTConfig,
	defaultTTSConfig,
	defaultVoiceConfig,
	nullableString,
	numberValue,
	requiredString,
	voiceToConfig,
	withDefaults
} from '$lib/backend/config'
import { batchUpdateConfig, getAllConfig } from '$lib/backend/http'
import type { STTConfig, TTSConfig, VoiceConfig } from '$lib/backend/types'
import { listInputDevices, vadGetDevice, vadSetDevice } from '$lib/commands'
import { vadStore } from '$lib/stores/vad.svelte'

type STTForm = Omit<STTConfig, 'language'> & { language: string }
type TTSForm = TTSConfig

let devices = $state<string[]>([])
let selectedDevice = $state<string>('')

let sttEnabled = $state(false)
let ttsEnabled = $state(false)
let stt = $state<STTForm>(sttToForm(defaultSTTConfig))
let tts = $state<TTSForm>({ ...defaultTTSConfig })
let lipsyncFps = $state(defaultVoiceConfig.lipsync_fps)
let configLoading = $state(true)
let configSaving = $state(false)
let configError = $state<string | null>(null)
let configSaved = $state(false)

const deviceOptions = $derived([
	{ value: '', label: '系統預設' },
	...devices.map((d) => ({ value: d, label: d }))
])

const ttsVoiceOptions = [
	{ value: 'alloy', label: 'alloy' },
	{ value: 'echo', label: 'echo' },
	{ value: 'fable', label: 'fable' },
	{ value: 'onyx', label: 'onyx' },
	{ value: 'nova', label: 'nova' },
	{ value: 'shimmer', label: 'shimmer' }
]

function sttToForm(value: Partial<STTConfig> | null | undefined): STTForm {
	const merged = withDefaults(defaultSTTConfig, value)
	return { ...merged, language: merged.language ?? '' }
}

async function loadDevices() {
	devices = await listInputDevices().catch(() => [])
	const current = await vadGetDevice().catch(() => null)
	selectedDevice = current ?? ''
}

async function loadVoiceConfig() {
	configLoading = true
	configError = null
	try {
		const config = await getAllConfig()
		const voice = configToVoice(config)
		sttEnabled = !!voice.stt
		ttsEnabled = !!voice.tts
		stt = sttToForm(voice.stt ?? defaultSTTConfig)
		tts = withDefaults(defaultTTSConfig, voice.tts)
		lipsyncFps = voice.lipsync_fps
	} catch (e) {
		configError = String(e)
	} finally {
		configLoading = false
	}
}

async function onDeviceChange(value: string) {
	selectedDevice = value
	await vadSetDevice(value === '' ? null : value).catch(() => {})
}

function cleanStt(): STTConfig | null {
	const apiKey = requiredString(stt.api_key)
	const baseUrl = requiredString(stt.base_url)
	const model = requiredString(stt.model)
	const temperature = numberValue(stt.temperature, Number.NaN)

	if (!apiKey) {
		configError = '請輸入 STT API key'
		return null
	}
	if (!baseUrl) {
		configError = '請輸入 STT base URL'
		return null
	}
	if (!model) {
		configError = '請輸入 STT model'
		return null
	}
	if (!Number.isFinite(temperature)) {
		configError = 'STT temperature 必須是數字'
		return null
	}

	return {
		api_key: apiKey,
		base_url: baseUrl,
		model,
		language: nullableString(stt.language),
		temperature
	}
}

function cleanTts(): TTSConfig | null {
	const apiKey = requiredString(tts.api_key)
	const baseUrl = requiredString(tts.base_url)
	const model = requiredString(tts.model)
	const voice = requiredString(tts.voice)

	if (!apiKey) {
		configError = '請輸入 TTS API key'
		return null
	}
	if (!baseUrl) {
		configError = '請輸入 TTS base URL'
		return null
	}
	if (!model) {
		configError = '請輸入 TTS model'
		return null
	}
	if (!voice) {
		configError = '請輸入 TTS voice'
		return null
	}

	return {
		api_key: apiKey,
		base_url: baseUrl,
		model,
		voice
	}
}

async function saveVoiceConfig() {
	configError = null
	configSaved = false
	const fps = numberValue(lipsyncFps, Number.NaN)
	if (!Number.isFinite(fps) || fps <= 0) {
		configError = 'lipsync FPS 必須大於 0'
		return
	}
	const cleanedStt = sttEnabled ? cleanStt() : null
	if (sttEnabled && !cleanedStt) return
	const cleanedTts = ttsEnabled ? cleanTts() : null
	if (ttsEnabled && !cleanedTts) return

	configSaving = true
	try {
		const payload: VoiceConfig = {
			stt: cleanedStt,
			tts: cleanedTts,
			lipsync_fps: fps
		}
		const config = await batchUpdateConfig(voiceToConfig(payload))
		const voice = configToVoice(config)
		sttEnabled = !!voice.stt
		ttsEnabled = !!voice.tts
		stt = sttToForm(voice.stt ?? defaultSTTConfig)
		tts = withDefaults(defaultTTSConfig, voice.tts)
		lipsyncFps = voice.lipsync_fps
		configSaved = true
	} catch (e) {
		configError = String(e)
	} finally {
		configSaving = false
	}
}

onMount(async () => {
	await Promise.all([vadStore.init(), loadDevices(), loadVoiceConfig()])
})
</script>

<div class="flex flex-col gap-3">
	<Setting.Card
		title="語音活動偵測 (VAD)"
		icon="icon-[material-symbols--mic-outline-rounded]"
		description="啟用後，系統將持續監聽麥克風，自動偵測語音片段並傳送至 AI 後端"
	>
		<div class="flex flex-col gap-3">
			<div class="flex items-center justify-between">
				<span class="text-muted-foreground text-xs">目前狀態</span>
				<Badge variant={vadStore.active ? 'primary' : 'muted'}>{vadStore.active ? '偵測中' : '已停止'}</Badge>
			</div>

			<Button variant={vadStore.active ? 'danger' : 'primary'} onclick={() => vadStore.toggle()} disabled={vadStore.busy}>
				{vadStore.active ? '停止偵測' : '開始偵測'}
			</Button>

			{#if vadStore.error}
				<p class="text-destructive text-xs">{vadStore.error}</p>
			{/if}
		</div>
	</Setting.Card>

	<Setting.Card
		title="麥克風設備"
		icon="icon-[material-symbols--settings-input-component-outline-rounded]"
		description="選擇語音偵測使用的輸入設備，預設使用系統設定的麥克風"
	>
		<Field label="輸入設備">
			<Select options={deviceOptions} bind:value={selectedDevice} placeholder="系統預設" onValueChange={onDeviceChange} />
		</Field>
	</Setting.Card>

	{#if configError}
		<p class="text-danger text-xs">{configError}</p>
	{/if}

	<Setting.Card
		title="STT 語音辨識"
		icon="icon-[material-symbols--record-voice-over-outline-rounded]"
		description="OpenAI Whisper compatible 語音辨識設定"
	>
		<div class="flex flex-col gap-3">
			<Switch id="stt-toggle" label="啟用 STT" bind:checked={sttEnabled} disabled={configLoading} />
			{#if configLoading}
				<p class="text-muted-foreground text-xs">載入中…</p>
			{:else if sttEnabled}
				<div class="grid gap-3 md:grid-cols-2">
					<Field label="API Key" required>
						<Input type="password" bind:value={stt.api_key} />
					</Field>
					<Field label="Base URL" required>
						<Input bind:value={stt.base_url} />
					</Field>
					<Field label="Model" required>
						<Input bind:value={stt.model} />
					</Field>
					<Field label="Language" hint="空白代表自動偵測">
						<Input bind:value={stt.language} placeholder="zh" />
					</Field>
					<Field label="Temperature" required>
						<Input type="number" step="0.1" bind:value={stt.temperature} />
					</Field>
				</div>
			{/if}
		</div>
	</Setting.Card>

	<Setting.Card
		title="TTS 語音合成"
		icon="icon-[material-symbols--volume-up-outline-rounded]"
		description="OpenAI TTS compatible 語音合成設定"
	>
		<div class="flex flex-col gap-3">
			<Switch id="tts-toggle" label="啟用 TTS" bind:checked={ttsEnabled} disabled={configLoading} />
			{#if !configLoading && ttsEnabled}
				<div class="grid gap-3 md:grid-cols-2">
					<Field label="API Key" required>
						<Input type="password" bind:value={tts.api_key} />
					</Field>
					<Field label="Base URL" required>
						<Input bind:value={tts.base_url} />
					</Field>
					<Field label="Model" required>
						<Input bind:value={tts.model} />
					</Field>
					<Field label="Voice" required>
						<Select options={ttsVoiceOptions} bind:value={tts.voice} />
					</Field>
				</div>
			{/if}
		</div>
	</Setting.Card>

	<Setting.Card
		title="Lipsync"
		icon="icon-[material-symbols--graphic-eq-rounded]"
		description="設定後端產生口型 viseme 的取樣幀率"
	>
		{#if !configLoading}
			<Field label="Lipsync FPS" required>
				<Input type="number" min="1" bind:value={lipsyncFps} />
			</Field>
		{/if}
	</Setting.Card>

	<div class="flex items-center gap-2">
		<Button variant="primary" onclick={saveVoiceConfig} disabled={configLoading || configSaving}>
			{configSaving ? '儲存中…' : '儲存設定'}
		</Button>
		{#if configSaved}
			<span class="text-primary text-xs">已儲存</span>
		{/if}
	</div>
</div>
