# css/CSS2/text/text-indent-014.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/text/text-indent-014.xht"
}
```

## style[0]

```css
<![CDATA[
    .container {
      font: 20px/1 Ahem;
      border: solid orange;
      width: 12em;
    }
    .outer {
      text-indent: 6em;
    }
    .inner {
      text-indent: 0;
    }
    .outer {
    }
    .outer:before {
      content: "BlockA";
      display: block;
    }

    .controlA, .controlB {
      background: red;
      height: 3em;
      width: 6em;
    }
    .controlA {
      margin-left: 6em;
      margin-bottom: -3em;
    }
    .controlB {
      margin-top: -3em;
    }
  ]]>
```

```json
{
  "errors": 3,
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
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
