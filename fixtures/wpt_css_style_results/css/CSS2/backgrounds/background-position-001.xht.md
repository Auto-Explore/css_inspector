# css/CSS2/backgrounds/background-position-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-001.xht"
}
```

## style[0]

```css
<![CDATA[
  html {font: 20px/1 Ahem;}

  body
  {
  background: url("support/1x1-lime.png") repeat-x 0 6.25ex;
  font-size: 2.5ex;
  margin: 1px 0px 8px 75px;
  }

  p#expected-results {font-family: serif;}

  img
  {
  left: 0px;
  position: absolute;
  top: 1px;
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
