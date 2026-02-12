# css/css-writing-modes/sizing-orthog-prct-htb-in-vlr-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-htb-in-vlr-002.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-lr;
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
      margin-left: 100px;
      margin-right: 100px;
    }

  div#auto-sized-vlr-containing-block
    {
      width: auto;
      /*
      'width: auto' causes the measurement of the orthogonal
      box's containing block to be indefinite
      */
    }

  div#ortho-block-htb
    {
      border: blue solid 3px;
      width: 50%;
      writing-mode: horizontal-tb;
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
