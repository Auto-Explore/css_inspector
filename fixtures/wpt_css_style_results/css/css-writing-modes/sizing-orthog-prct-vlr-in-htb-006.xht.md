# css/css-writing-modes/sizing-orthog-prct-vlr-in-htb-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-vlr-in-htb-006.xht"
}
```

## style[0]

```css
<![CDATA[
  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin-bottom: 0px;
      margin-top: 0px;
    }

  div#auto-sized-htb-containing-block
    {
      height: auto;
      /*
      'height: auto' causes the measurement of the orthogonal
      box's containing block to be indefinite
      */
    }

  div#ortho-block-vlr
    {
      border: blue solid 3px;
      height: 50%;
      writing-mode: vertical-lr;
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
