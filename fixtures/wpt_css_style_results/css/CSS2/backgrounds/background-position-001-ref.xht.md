# css/CSS2/backgrounds/background-position-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-position-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  font: 40px/1 serif;
  margin: 1px 0px 8px 75px;
  }

  p#expected-results {font-family: serif;}

  img
  {
  left: 0px;
  position: absolute;
  top: 1px;
  }

  div
  {
  border-top: lime solid 1px;
  left: 0px;
  position: absolute;
  top: 200px;
  width: 100%;
  z-index: -1;
  }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
