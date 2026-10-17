<script lang="ts">
import { Badge, Button, Field, Input, Select, Setting, Switch } from '@waifu/ui'
import { onMount } from 'svelte'
import {
	configToVoice,
	defaultDeepLConfig,
	defaultGPTSoVITSTTSConfig,
	defaultOpenAITTSConfig,
	defaultSTTConfig,
	defaultVoiceConfig,
	nullableString,
	numberValue,
	requiredString,
	voiceToConfig,
	withDefaults
} from '$lib/backend/config'
import { batchUpdateConfig, getAllConfig } from '$lib/backend/http'
import type {
	DeepLConfig,
	GPTSoVITSTTSConfig,
	OpenAITTSConfig,
	STTConfig,
	TTSBackend,
	TTSConfig,
	VoiceConfig
} from '$lib/backend/types'
import { listInputDevices, vadGetDevice, vadSetDevice } from '$lib/commands'
import { vadStore } from '$lib/stores/vad.svelte'

type STTForm = Omit<STTConfig, 'language'> & { language: string }

let devices = $state<string[]>([])
let selectedDevice = $state<string>('')

let sttEnabled = $state(false)
let ttsEnabled = $state(false)
let translationEnabled = $state(false)
let stt = $state<STTForm>(sttToForm(defaultSTTConfig))
// 各 backend 各自保留表單內容，切換 backend 時不會遺失
let ttsBackend = $state<TTSBackend>('openai')
let openaiTts = $state<OpenAITTSConfig>({ ...defaultOpenAITTSConfig })
let gptsovitsTts = $state<GPTSoVITSTTSConfig>({ ...defaultGPTSoVITSTTSConfig })
let translation = $state<DeepLConfig>({ ...defaultDeepLConfig })
let lipsyncFps = $state(defaultVoiceConfig.lipsync_fps)
let configLoading = $state(true)
let configSaving = $state(false)
let configError = $state<string | null>(null)
let configSaved = $state(false)

const deviceOptions = $derived([
	{ value: '', label: '系統預設' },
	...devices.map((d) => ({ value: d, label: d }))
])

const ttsBackendOptions = [
	{ value: 'openai', label: 'OpenAI compatible' },
	{ value: 'gptsovits', label: 'GPT-SoVITS' }
]

const streamingModeOptions = [
	{ value: '0', label: '停用' },
	{ value: '1', label: '最佳品質' },
	{ value: '2', label: '中等' },
	{ value: '3', label: '最快' }
]

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

function applyVoice(voice: VoiceConfig) {
	sttEnabled = !!voice.stt
	ttsEnabled = !!voice.tts
	translationEnabled = !!voice.translation
	stt = sttToForm(voice.stt ?? defaultSTTConfig)
	if (voice.tts?.backend === 'gptsovits') {
		gptsovitsTts = withDefaults(defaultGPTSoVITSTTSConfig, voice.tts)
	} else if (voice.tts) {
		openaiTts = withDefaults(defaultOpenAITTSConfig, voice.tts)
	}
	ttsBackend = voice.tts?.backend ?? 'openai'
	translation = withDefaults(defaultDeepLConfig, voice.translation)
	lipsyncFps = voice.lipsync_fps
}

async function loadVoiceConfig() {
	configLoading = true
	configError = null
	try {
		applyVoice(configToVoice(await getAllConfig()))
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

function cleanRetry(form: TTSConfig) {
	const timeoutRetries = numberValue(form.timeout_retries, Number.NaN)
	const retryBackoff = numberValue(form.retry_backoff_s, Number.NaN)
	if (!Number.isInteger(timeoutRetries) || timeoutRetries < 0) {
		configError = 'TTS 逾時重試次數必須是 0 以上的整數'
		return null
	}
	if (!Number.isFinite(retryBackoff) || retryBackoff < 0) {
		configError = 'TTS 重試間隔必須是 0 以上的數字'
		return null
	}
	return { timeout_retries: timeoutRetries, retry_backoff_s: retryBackoff }
}

function cleanOpenAITts(): OpenAITTSConfig | null {
	const apiKey = requiredString(openaiTts.api_key)
	const baseUrl = requiredString(openaiTts.base_url)
	const model = requiredString(openaiTts.model)
	const voice = requiredString(openaiTts.voice)
	const responseFormat = requiredString(openaiTts.response_format)
	const speed = numberValue(openaiTts.speed, Number.NaN)

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
	if (!Number.isFinite(speed) || speed <= 0) {
		configError = 'TTS speed 必須大於 0'
		return null
	}
	const retry = cleanRetry(openaiTts)
	if (!retry) return null

	return {
		backend: 'openai',
		api_key: apiKey,
		base_url: baseUrl,
		model,
		voice,
		response_format: responseFormat || defaultOpenAITTSConfig.response_format,
		speed,
		...retry
	}
}

function cleanGPTSoVITSTts(): GPTSoVITSTTSConfig | null {
	const form = gptsovitsTts
	const baseUrl = requiredString(form.base_url)
	if (!baseUrl) {
		configError = '請輸入 GPT-SoVITS base URL'
		return null
	}
	const numbers = {
		batch_size: numberValue(form.batch_size, Number.NaN),
		speed_factor: numberValue(form.speed_factor, Number.NaN),
		streaming_mode: numberValue(form.streaming_mode, Number.NaN),
		top_k: numberValue(form.top_k, Number.NaN),
		top_p: numberValue(form.top_p, Number.NaN),
		temperature: numberValue(form.temperature, Number.NaN),
		timeout: numberValue(form.timeout, Number.NaN)
	}
	const invalid = Object.entries(numbers).find(([, v]) => !Number.isFinite(v))
	if (invalid) {
		configError = `GPT-SoVITS ${invalid[0]} 必須是數字`
		return null
	}
	const retry = cleanRetry(form)
	if (!retry) return null

	return {
		backend: 'gptsovits',
		base_url: baseUrl,
		ref_audio_path: requiredString(form.ref_audio_path),
		prompt_text: form.prompt_text ?? '',
		prompt_lang: requiredString(form.prompt_lang),
		text_lang: requiredString(form.text_lang),
		text_split_method: requiredString(form.text_split_method),
		...numbers,
		...retry
	}
}

function cleanTranslation(): DeepLConfig | null {
	const apiKey = requiredString(translation.api_key)
	const targetLang = requiredString(translation.target_lang).toUpperCase()
	if (!apiKey) {
		configError = '請輸入 DeepL API key'
		return null
	}
	if (!targetLang) {
		configError = '請輸入翻譯目標語言'
		return null
	}
	return { api_key: apiKey, target_lang: targetLang }
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
	const cleanedTts = ttsEnabled
		? ttsBackend === 'gptsovits'
			? cleanGPTSoVITSTts()
			: cleanOpenAITts()
		: null
	if (ttsEnabled && !cleanedTts) return
	const cleanedTranslation = translationEnabled ? cleanTranslation() : null
	if (translationEnabled && !cleanedTranslation) return

	configSaving = true
	try {
		const payload: VoiceConfig = {
			stt: cleanedStt,
			tts: cleanedTts,
			lipsync_fps: fps,
			translation: cleanedTranslation
		}
		applyVoice(configToVoice(await batchUpdateConfig(voiceToConfig(payload))))
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
		description="OpenAI TTS compatible 或本機 GPT-SoVITS 語音合成設定"
	>
		<div class="flex flex-col gap-3">
			<Switch id="tts-toggle" label="啟用 TTS" bind:checked={ttsEnabled} disabled={configLoading} />
			{#if !configLoading && ttsEnabled}
				<Field label="Backend" required>
					<Select
						options={ttsBackendOptions}
						bind:value={ttsBackend}
						onValueChange={(value) => (ttsBackend = value as TTSBackend)}
					/>
				</Field>
				{#if ttsBackend === 'openai'}
					<div class="grid gap-3 md:grid-cols-2">
						<Field label="API Key" required>
							<Input type="password" bind:value={openaiTts.api_key} />
						</Field>
						<Field label="Base URL" required>
							<Input bind:value={openaiTts.base_url} />
						</Field>
						<Field label="Model" required>
							<Input bind:value={openaiTts.model} />
						</Field>
						<Field label="Voice" required>
							<Select options={ttsVoiceOptions} bind:value={openaiTts.voice} />
						</Field>
						<Field label="Response Format">
							<Input bind:value={openaiTts.response_format} placeholder="mp3" />
						</Field>
						<Field label="Speed" required>
							<Input type="number" step="0.1" min="0.25" bind:value={openaiTts.speed} />
						</Field>
						<Field label="逾時重試次數">
							<Input type="number" min="0" bind:value={openaiTts.timeout_retries} />
						</Field>
						<Field label="重試間隔 (秒)">
							<Input type="number" min="0" step="0.1" bind:value={openaiTts.retry_backoff_s} />
						</Field>
					</div>
				{:else}
					<div class="grid gap-3 md:grid-cols-2">
						<Field label="Base URL" required>
							<Input bind:value={gptsovitsTts.base_url} />
						</Field>
						<Field label="參考音訊路徑" hint="GPT-SoVITS 伺服器上的檔案路徑">
							<Input bind:value={gptsovitsTts.ref_audio_path} />
						</Field>
						<Field label="參考音訊文字">
							<Input bind:value={gptsovitsTts.prompt_text} />
						</Field>
						<Field label="參考音訊語言">
							<Input bind:value={gptsovitsTts.prompt_lang} placeholder="zh" />
						</Field>
						<Field label="合成語言">
							<Input bind:value={gptsovitsTts.text_lang} placeholder="zh" />
						</Field>
						<Field label="切句方式">
							<Input bind:value={gptsovitsTts.text_split_method} placeholder="cut5" />
						</Field>
						<Field label="Streaming Mode">
							<Select
								options={streamingModeOptions}
								value={String(gptsovitsTts.streaming_mode)}
								onValueChange={(value) => (gptsovitsTts.streaming_mode = Number(value))}
							/>
						</Field>
						<Field label="Speed Factor">
							<Input type="number" step="0.1" bind:value={gptsovitsTts.speed_factor} />
						</Field>
						<Field label="Batch Size">
							<Input type="number" min="1" bind:value={gptsovitsTts.batch_size} />
						</Field>
						<Field label="Top K">
							<Input type="number" min="1" bind:value={gptsovitsTts.top_k} />
						</Field>
						<Field label="Top P">
							<Input type="number" step="0.05" min="0" max="1" bind:value={gptsovitsTts.top_p} />
						</Field>
						<Field label="Temperature">
							<Input type="number" step="0.1" bind:value={gptsovitsTts.temperature} />
						</Field>
						<Field label="逾時 (秒)">
							<Input type="number" min="1" bind:value={gptsovitsTts.timeout} />
						</Field>
						<Field label="逾時重試次數">
							<Input type="number" min="0" bind:value={gptsovitsTts.timeout_retries} />
						</Field>
						<Field label="重試間隔 (秒)">
							<Input type="number" min="0" step="0.1" bind:value={gptsovitsTts.retry_backoff_s} />
						</Field>
					</div>
				{/if}
			{/if}
		</div>
	</Setting.Card>

	<Setting.Card
		title="翻譯"
		icon="icon-[material-symbols--translate-rounded]"
		description="設定後於 TTS 合成前以 DeepL 將回覆翻譯為目標語言"
	>
		<div class="flex flex-col gap-3">
			<Switch id="translation-toggle" label="啟用 DeepL 翻譯" bind:checked={translationEnabled} disabled={configLoading} />
			{#if !configLoading && translationEnabled}
				<div class="grid gap-3 md:grid-cols-2">
					<Field label="API Key" required>
						<Input type="password" bind:value={translation.api_key} />
					</Field>
					<Field label="目標語言" hint="DeepL 語言代碼，例如 JA、EN-US" required>
						<Input bind:value={translation.target_lang} placeholder="JA" />
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
