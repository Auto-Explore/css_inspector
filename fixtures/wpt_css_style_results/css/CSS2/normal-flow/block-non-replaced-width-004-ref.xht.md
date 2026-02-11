# css/CSS2/normal-flow/block-non-replaced-width-004-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/block-non-replaced-width-004-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body {direction: rtl;}

  p {direction: ltr;}

  div
  {
  height: 30px;
  width: 150px;
  }

  div#orange {background-color: orange;}

  div#blue
  {
  background-color: blue;
  position: absolute;
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
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
