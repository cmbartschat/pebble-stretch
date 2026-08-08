module.exports = [
  {
    type: 'section',
    items: [
      {
        type: 'color',
        label: 'Background Color',
        messageKey: 'CONFIG_BACKGROUND_COLOR',
        defaultValue: '#000000',
      },
      {
        type: 'color',
        label: 'Digit 1 Color',
        messageKey: 'CONFIG_DIGIT_0_COLOR',
        defaultValue: '#ffffff',
      },
      {
        type: 'color',
        label: 'Digit 2 Color',
        messageKey: 'CONFIG_DIGIT_1_COLOR',
        defaultValue: '#ffffff',
      },
      {
        type: 'color',
        label: 'Digit 3 Color',
        messageKey: 'CONFIG_DIGIT_2_COLOR',
        defaultValue: '#ffffff',
      },
      {
        type: 'color',
        label: 'Digit 4 Color',
        messageKey: 'CONFIG_DIGIT_3_COLOR',
        defaultValue: '#ffffff',
      },
    ],
  },
  {
    type: 'submit',
    defaultValue: 'Save',
  },
]
