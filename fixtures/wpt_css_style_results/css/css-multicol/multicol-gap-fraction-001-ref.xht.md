# css/css-multicol/multicol-gap-fraction-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-gap-fraction-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body > div
  {
  background-color: gray;
  font: 1.25em/1 Ahem;
  height: 4em;
  position: relative;
  width: 14.5em;
  }

  p, div > div
  {
  background-color: blue;
  height: 1em;
  left: 0;
  margin: 0;
  position: absolute;
  top: 0;
  width: 4em;
  }

  div > div
  {
  background-color: black;
  height: 4em;
  }

  p {left: 7.5em;}

  p + p
  {
  left: 8.5em;
  top: 1em;
  }

  p + p + p
  {
  left: 9.5em;
  top: 2em;
  }

  p + p + p + p
  {
  left: 10.5em;
  top: 3em;
  }
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
