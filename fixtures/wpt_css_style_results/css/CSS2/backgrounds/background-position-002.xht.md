# css/CSS2/backgrounds/background-position-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-002.xht"
}
```

## style[0]

```css
<![CDATA[
  html {font: 20px/1 Ahem;}

  body
  {
  background: url("support/1x1-lime.png") repeat-y 12.5ex 0;
  font-size: 2.5ex;
  margin: 80px 0px 8px 8px;
  }

  p#expected-results {font-family: serif;}

  img
  {
  left: 0px;
  position: absolute;
  top: 42px;
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
