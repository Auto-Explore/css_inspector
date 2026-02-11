# css/css-fonts/font-size-adjust-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-size-adjust-006.xht"
}
```

## style[0]

```css
<![CDATA[
  div#test-percent-value
  {
  color: green;
  font: 100px/1 Ahem; /* computes to 100px/100px */
  font-size-adjust: 50%;
  }

  div#reference-overlapped-red
  {
  background-color: red;
  bottom: 100px;
  height: 100px;
  position: relative;
  width: 100px;
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
