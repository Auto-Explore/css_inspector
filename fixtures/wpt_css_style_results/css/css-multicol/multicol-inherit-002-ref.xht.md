# css/css-multicol/multicol-inherit-002-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-inherit-002-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  div
  {
  background-color: blue;
  font: 1.25em/1 serif;
  margin: 1em;
  padding: 0em 1em 1em;
  width: 30em;
  }

  img {vertical-align: top;}

  img.column-gap
  {
  margin-left: -2em;
  margin-right: 3em;
  vertical-align: bottom;
  }

  img#last
  {
  margin-left: -2em;
  vertical-align: bottom;
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
