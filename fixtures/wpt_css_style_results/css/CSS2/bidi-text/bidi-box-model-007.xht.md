# css/CSS2/bidi-text/bidi-box-model-007.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/bidi-box-model-007.xht"
}
```

## style[0]

```css
<![CDATA[
      div {
      border: 5px solid gray;
      border-color: orange purple teal yellow;
      text-align: left;
      width: 10em;
      margin-bottom: 1em;
      }

      .rtol {
      direction: rtl;
      unicode-bidi: normal;
      }

      p {text-align: left;}
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
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
