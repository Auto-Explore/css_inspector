# css/CSS2/bidi-text/bidi-box-model-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/bidi-box-model-006.xht"
}
```

## style[0]

```css
<![CDATA[
      span {
      border: 5px solid gray;
      border-color: orange purple teal yellow;
      }

      .rtol {
      direction: rtl;
      unicode-bidi: bidi-override;
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
