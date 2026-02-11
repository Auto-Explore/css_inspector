# css/CSS2/box-display/block-in-inline-relpos-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/block-in-inline-relpos-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      margin: 1em;
      font: 20px/1 Ahem;
      border: solid silver;
      width: 4em;
      color: aqua;
      background: fuchsia;
    }

    .controlC {
      color: orange;
      background: orange;
      width: 2em;
      margin-left: -2em;
      border-left: 2em solid blue
    }

    .controlB {
      color: yellow;
    }
    .controlC {
      margin-left: 0;
    }
  ]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
