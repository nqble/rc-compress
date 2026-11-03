// The user's own Quick Compress groups (Settings -> Right-click Menu): file types with the preset
// "Quick Compress" uses for them, checked in order before the built-in video, image and audio
// presets. The matching itself is in `utils/quickCompressGroups.ts`.
import { SelectItem } from '@heroui/react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useSnapshot } from 'valtio'

import Button from '@/components/Button'
import Icon from '@/components/Icon'
import Select from '@/components/Select'
import TextInput from '@/components/TextInput'
import Tooltip from '@/components/Tooltip'
import { settingsStore } from '@/stores/settingsStore'
import { inputExtensions } from '@/types/compression'
import { DEFAULT_PRESET_CHOICE, type PresetMediaType } from '@/types/preset'
import {
  parseExtensions,
  type QuickCompressGroup,
} from '@/utils/quickCompressGroups'
import PresetChoiceSelect from './PresetChoiceSelect'
import { SettingsSection } from './SettingsLayout'

const MEDIA_TYPES: PresetMediaType[] = ['video', 'image', 'audio']

/** Changes one group, by its id */
function updateGroup(id: string, change: Partial<QuickCompressGroup>) {
  settingsStore.quickCompressGroups = settingsStore.quickCompressGroups.map(
    (group) => (group.id === id ? { ...group, ...change } : group),
  )
}

function removeGroup(id: string) {
  settingsStore.quickCompressGroups = settingsStore.quickCompressGroups.filter(
    (group) => group.id !== id,
  )
}

function addGroup(name: string) {
  const group: QuickCompressGroup = {
    id: crypto.randomUUID(),
    name,
    mediaType: 'video',
    extensions: [],
    presetId: DEFAULT_PRESET_CHOICE,
  }
  settingsStore.quickCompressGroups = [
    ...settingsStore.quickCompressGroups,
    group,
  ]
}

/**
 * The file types of a group, typed freely ("mov, mkv") and read when the field is left: only types
 * of the group's kind of media are kept
 */
function ExtensionsField({ group }: { group: QuickCompressGroup }) {
  const { t } = useTranslation()
  const [text, setText] = useState(group.extensions.join(', '))

  const save = () => {
    const extensions = parseExtensions(text, inputExtensions[group.mediaType])
    updateGroup(group.id, { extensions })
    setText(extensions.join(', '))
  }

  return (
    <TextInput
      label={t('settings.rightClickMenu.groups.fileTypes')}
      placeholder={t('settings.rightClickMenu.groups.fileTypesPlaceholder', {
        context: group.mediaType,
      })}
      value={text}
      onValueChange={setText}
      onBlur={save}
    />
  )
}

function GroupCard({ group }: { group: QuickCompressGroup }) {
  const { t } = useTranslation()

  return (
    <div className="rounded-xl border border-zinc-200 dark:border-zinc-700 p-3 flex flex-col gap-3">
      <div className="flex items-end gap-2">
        <TextInput
          label={t('settings.rightClickMenu.groups.name')}
          value={group.name}
          onValueChange={(name) => updateGroup(group.id, { name })}
        />
        <Tooltip
          content={t('settings.rightClickMenu.groups.remove')}
          aria-label={t('settings.rightClickMenu.groups.remove')}
        >
          <Button
            isIconOnly
            size="sm"
            variant="flat"
            color="danger"
            aria-label={t('settings.rightClickMenu.groups.removeNamed', {
              name: group.name,
            })}
            onPress={() => removeGroup(group.id)}
          >
            <Icon name="trash" size={18} />
          </Button>
        </Tooltip>
      </div>
      <div className="grid grid-cols-3 gap-3 items-end">
        <Select
          label={t('settings.rightClickMenu.groups.kind')}
          selectedKeys={[group.mediaType]}
          disallowEmptySelection
          onSelectionChange={(keys) => {
            const [key] = Array.from(keys)
            if (!key) return
            // Its types and preset were for the other kind of media
            updateGroup(group.id, {
              mediaType: key as PresetMediaType,
              extensions: [],
              presetId: DEFAULT_PRESET_CHOICE,
            })
          }}
        >
          {MEDIA_TYPES.map((mediaType) => (
            <SelectItem key={mediaType}>
              {t('settings.rightClickMenu.groups.kind', { context: mediaType })}
            </SelectItem>
          ))}
        </Select>
        <ExtensionsField key={`${group.id}-${group.mediaType}`} group={group} />
        <PresetChoiceSelect
          aria-label={t('settings.rightClickMenu.groups.preset', {
            name: group.name,
          })}
          mediaType={group.mediaType}
          value={group.presetId}
          onChange={(presetId) => updateGroup(group.id, { presetId })}
          classNames={{ popoverContent: 'settings-dialog' }}
        />
      </div>
    </div>
  )
}

function QuickCompressGroups() {
  const { t } = useTranslation()
  const { quickCompressGroups } = useSnapshot(settingsStore)

  return (
    <SettingsSection title={t('settings.rightClickMenu.groups.title')}>
      <p className="text-sm text-gray-500 px-1">
        {t('settings.rightClickMenu.groups.description')}
      </p>
      {quickCompressGroups.map((group) => (
        <GroupCard key={group.id} group={group as QuickCompressGroup} />
      ))}
      <div>
        <Button
          size="sm"
          onPress={() => addGroup(t('settings.rightClickMenu.groups.newName'))}
        >
          <Icon name="plus" size={16} />
          {t('settings.rightClickMenu.groups.add')}
        </Button>
      </div>
    </SettingsSection>
  )
}

export default QuickCompressGroups
