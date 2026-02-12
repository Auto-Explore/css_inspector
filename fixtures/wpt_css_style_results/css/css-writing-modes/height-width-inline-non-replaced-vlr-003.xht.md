# css/css-writing-modes/height-width-inline-non-replaced-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/height-width-inline-non-replaced-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div#block
    {
      font: 100px/1 Ahem; /* computes to 100px/100px */
      height: 2em;
      width: 2em;
      writing-mode: vertical-lr;
    }

  div#inline
    {
      background-color: red;
      color: red;
      display: inline;
      height: 2em;
      width: 2em;
    }

  div#overlapping-green
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
