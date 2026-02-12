# css/css-multicol/multicol-width-count-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-multicol/multicol-width-count-002.xht"
}
```

## style[0]

```css
<![CDATA[
  html {background-color: white;}

  body
  {
  background-color: black;
  font: 1.25em/1 Ahem;
  padding-right: 1em;
  width: 20em;
  }

  div
  {
  background-color: yellow;
  color: black;
  column-count: 4;
  column-gap: 0;
  column-width: 4em; /* would create 5 columns if column-count was not acting as a max */
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
