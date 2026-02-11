# css/css-writing-modes/line-box-direction-srl-051.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/line-box-direction-srl-051.xht"
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
      vertical-align: top;
  /*
  Why 'vertical-align: top' ?
  See
  http://lists.w3.org/Archives/Public/public-css-testsuite/2014Dec/0013.html
  for explanations
  */
      writing-mode: sideways-rl;
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
