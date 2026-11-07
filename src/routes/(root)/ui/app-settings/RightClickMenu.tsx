// Settings of the file manager's right-click menu (Windows Explorer for now; the entries are added
// by the installer, see `src-tauri/windows/hooks.nsh` and `explorer_menu.rs`)
import { Trans, useTranslation } from 'react-i18next'
import { useSnapshot } from 'valtio'

import InfoTooltip from '@/components/InfoTooltip'
import SettingLink from '@/components/SettingLink'
import { canWatchedFolderSave } from '@/hooks/useAutoCompressSync'
import { presetsStore } from '@/stores/presetsStore'
import { settingsStore } from '@/stores/settingsStore'
import type { PresetMediaType } from '@/types/preset'
import {
  type PresetsState,
  QUICK_COMPRESS_PRESET_SETTINGS,
} from '@/utils/presetOutput'
import PresetChoiceSelect from './PresetChoiceSelect'
import QuickCompressGroups from './QuickCompressGroups'
import { SettingsRow, SettingsSection } from './SettingsLayout'

const MEDIA_TYPES: PresetMediaType[] = ['video', 'image', 'audio']

const SETTING_IDS = {
  video: 'quickCompressVideoPreset',
  image: 'quickCompressImagePreset',
  audio: 'quickCompressAudioPreset',
} as const satisfies Record<PresetMediaType, string>

/** Picks the preset "Quick Compress" uses for one type of media (Default unless chosen) */
function QuickCompressPresetSelect({
  mediaType,
}: {
  mediaType: PresetMediaType
}) {
  const { t } = useTranslation()
  const settings = useSnapshot(settingsStore)
  const settingKey = QUICK_COMPRESS_PRESET_SETTINGS[mediaType]

  return (
    <PresetChoiceSelect
      aria-label={t('settings.rightClickMenu.preset.title', {
        context: mediaType,
      })}
      mediaType={mediaType}
      value={settings[settingKey]}
      onChange={(choice) => {
        settingsStore[settingKey] = choice
      }}
      className="w-[220px]"
      classNames={{ popoverContent: 'settings-dialog' }}
    />
  )
}

/**
 * Whether every Quick Compress preset saves without asking where: nothing can ask, so media of a
 * preset that asks waits in the queue to be saved by hand
 */
function useQuickCompressSaves() {
  const settings = useSnapshot(settingsStore)
  const presetsState = useSnapshot(presetsStore)
  const presetIds = {
    video: settings.quickCompressVideoPresetId,
    image: settings.quickCompressImagePresetId,
    audio: settings.quickCompressAudioPresetId,
  }
  return canWatchedFolderSave(
    { presetIds },
    settings as typeof settingsStore,
    presetsState as PresetsState,
  )
}

/** The built-in presets: one per kind of media, used when no group of the user's matches */
function QuickCompressPresets() {
  const { t } = useTranslation()
  const savesAutomatically = useQuickCompressSaves()

  return (
    <SettingsSection title={t('settings.rightClickMenu.quickCompress.title')}>
      <p className="text-sm text-gray-500 px-1">
        {t('settings.rightClickMenu.quickCompress.description')}
      </p>
      {savesAutomatically ? null : (
        <div className="flex items-center gap-2 px-1 text-sm">
          <InfoTooltip
            content={
              <Trans
                i18nKey="settings.rightClickMenu.quickCompress.needsSaveLocation"
                components={{
                  setting: (
                    <SettingLink target={{ settingId: 'saveLocation' }} />
                  ),
                }}
              />
            }
          />
          {t('settings.rightClickMenu.quickCompress.asksWhereToSave')}
        </div>
      )}
      {MEDIA_TYPES.map((mediaType) => (
        <SettingsRow
          key={mediaType}
          settingId={SETTING_IDS[mediaType]}
          title={t('settings.rightClickMenu.preset.title', {
            context: mediaType,
          })}
          description={t('settings.rightClickMenu.preset.description', {
            context: mediaType,
          })}
        >
          <QuickCompressPresetSelect mediaType={mediaType} />
        </SettingsRow>
      ))}
    </SettingsSection>
  )
}

function RightClickMenu() {
  return (
    <>
      <QuickCompressPresets />
      <QuickCompressGroups />
    </>
  )
}

export default RightClickMenu
