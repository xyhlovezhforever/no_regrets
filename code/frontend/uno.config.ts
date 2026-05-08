import { defineConfig, presetUno } from 'unocss'
import presetWeapp from 'unocss-preset-weapp'
import transformerDirectives from '@unocss/transformer-directives'
import transformerVariantGroup from '@unocss/transformer-variant-group'

const isH5 = process.env.UNI_PLATFORM === 'h5'
const isApp = process.env.UNI_PLATFORM === 'app'
const WeappPreset = presetWeapp

export default defineConfig({
  presets: [
    isH5 || isApp
      ? presetUno()
      : WeappPreset({
          isH5,
          platform: 'uniapp',
          whRpx: false,
        }),
  ],
  transformers: [
    transformerDirectives(),
    transformerVariantGroup(),
  ],
  shortcuts: {
    'flex-center': 'flex justify-center items-center',
    'flex-between': 'flex justify-between items-center',
    'flex-around': 'flex justify-around items-center',
    'flex-col-center': 'flex flex-col justify-center items-center',
  },
})

