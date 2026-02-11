# css/CSS2/box-display/block-in-inline-relpos-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/block-in-inline-relpos-002.xht"
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
    .relpos {
      position: relative;
      display: inline;
      color: yellow;
      left: 2em;
    }
    .block, .controlC {
      color: orange;
      background: orange;
      width: 2em;
      border-left: 2em solid blue;
    }
    .block {
      margin-left: -2em;
      height: 10px;
    }

    .float {
      width: 2em;
      height: 10px;
    }
    .float.L {
      float: left;
      background: orange;
      color: orange;
    }
    .float.R {
      float: right;
      background: blue;
      color: blue;
      /* Back position to fill the hole left by relpos'ing .float.L */
      position: relative;
      right: 4em;
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
  "errors": 6,
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
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
