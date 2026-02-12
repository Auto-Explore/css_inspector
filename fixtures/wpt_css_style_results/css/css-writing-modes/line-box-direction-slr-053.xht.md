# css/css-writing-modes/line-box-direction-slr-053.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/line-box-direction-slr-053.xht"
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
      border-bottom: blue solid 1em;
      border-right: blue solid 1em;
      display: inline-block;
      height: 8em; /* Each line box has an inline-size of 8em */
      writing-mode: sideways-lr;
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
