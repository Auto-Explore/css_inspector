# css/css-multicol/multicol-nested-margin-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-nested-margin-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body {margin: 0em;}

  div#rel-pos-wrapper
  {
  background-color: yellow;
  font: 1.25em/1 serif;
  height: 3em;
  margin: 1em;
  position: relative;
  width: 32em;
  }

  div > div
  {
  font-size: 1em;
  height: 3em;
  position: absolute;
  top: 0em;
  width: 2em;
  }

  div#black1
  {
  background-color: black;
  left: 1em;
  }

  div#black2
  {
  background-color: black;
  left: 4em;
  }

  div#black3
  {
  background-color: black;
  height: 2em;
  left: 7em;
  }

  div#pink1
  {
  background-color: pink;
  left: 12em;
  }

  div#pink2
  {
  background-color: pink;
  left: 15em;
  }

  div#pink3
  {
  background-color: pink;
  height: 2em;
  left: 18em;
  }

  div#blue1
  {
  background-color: blue;
  left: 23em;
  }

  div#blue2
  {
  background-color: blue;
  left: 26em;
  }

  div#blue3
  {
  background-color: blue;
  height: 2em;
  left: 29em;
  }
   ]]>
```

```json
{
  "errors": 5,
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
    },
    {
      "message": "Invalid value for property “background-color”.",
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
