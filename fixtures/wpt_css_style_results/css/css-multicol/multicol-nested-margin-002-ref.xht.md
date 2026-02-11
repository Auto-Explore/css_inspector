# css/css-multicol/multicol-nested-margin-002-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-margin-002-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 0em;}

  div
  {
  background-color: yellow;
  font: 1.25em/1 serif;
  height: 5em;
  position: relative;
  width: 41em;
  }

  div > div
  {
  background-color: blue;
  font: inherit;
  height: 3em;
  position: absolute;
  top: 0em;
  width: 2em
  }

  div#first-blue
  {
  background-color: blue;
  left: 29em;
  }

  div#second-blue
  {
  background-color: blue;
  left: 33em;
  }

  div#third-blue
  {
  background-color: blue;
  height: 2em;
  left: 37em;
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
