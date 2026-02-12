# css/css-writing-modes/sizing-orthog-vrl-in-htb-016.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vrl-in-htb-016.xht"
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

  div#ortho-block-vrl
    {
      border: blue solid 35px;
      height: auto;
      writing-mode: vertical-rl;
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
