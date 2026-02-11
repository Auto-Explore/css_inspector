# css/css-multicol/multicol-margin-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-margin-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div#test-multi-column-red
  {
  background-color: red;
  color: white;
  font: 3.125em/1 Ahem; /* equivalent to 50px/50px Ahem */
  height: 4em;
  margin-top: 0em;
  width: 4em;

  column-count: 2;
  column-fill: balance;
  column-gap: 0em;
  }

  span
  {
  display: inline-block;
  margin-top: 2em;
  }

  div#reference-overlapping-green
  {
  background-color: green;
  bottom: 200px;
  height: 100px;
  position: relative;
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
