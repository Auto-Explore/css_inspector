# css/css-writing-modes/line-box-direction-vlr-013.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/line-box-direction-vlr-013.xht"
}
```

## style[0]

```css
<![CDATA[
  body
    {
      color: yellow;
      font: 20px/1 Ahem;
    }

  div#inline-block
    {
      background-color: blue;
      border-right: blue solid 1em;
      border-top: blue solid 1em;
      display: inline-block;
      height: 8em; /* Each line box has an inline-size of 8em */
      writing-mode: vertical-lr;
    }

  span
    {
      border-left: blue solid 1em;
      display: block;
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
