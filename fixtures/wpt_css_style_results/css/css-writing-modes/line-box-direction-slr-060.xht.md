# css/css-writing-modes/line-box-direction-slr-060.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/line-box-direction-slr-060.xht"
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

  ul
    {
      background-color: blue;
      border: blue solid 1em;
      height: 7em; /* Each line box has an inline-size of 7em */
      list-style: none outside none;
      margin: 0em;
      padding-bottom: 0em; /* overriding default padding-end: 40px in several browsers */
      writing-mode: sideways-lr;
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
