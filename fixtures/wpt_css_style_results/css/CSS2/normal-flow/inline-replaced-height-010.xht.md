# css/CSS2/normal-flow/inline-replaced-height-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/normal-flow/inline-replaced-height-010.xht"
}
```

## style[0]

```css
<![CDATA[
  div#overlapped-red-test
  {
  position: absolute;
  z-index: -1;
  }

  img
  {
  height: auto;
  max-width: 6.25em;
  width: 12.5em;
  }

  div#overlapping-green-reference
  {
  background-color: green;
  height: 100px;
  width: 100px;
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
