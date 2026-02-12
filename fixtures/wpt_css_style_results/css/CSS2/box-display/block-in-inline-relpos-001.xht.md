# css/CSS2/box-display/block-in-inline-relpos-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/box-display/block-in-inline-relpos-001.xht"
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
      color: yellow;
      left: 2em;
      display: inline;
    }
    .block, .controlC {
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
