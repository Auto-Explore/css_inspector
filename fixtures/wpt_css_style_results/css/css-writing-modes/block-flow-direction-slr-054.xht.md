# css/css-writing-modes/block-flow-direction-slr-054.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/block-flow-direction-slr-054.xht"
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
      display: inline-block;
      height: 8em;
      vertical-align: top;
  /*
  Why 'vertical-align: top' ?
  See
  http://lists.w3.org/Archives/Public/public-css-testsuite/2014Dec/0013.html
  for explanations
  */
      writing-mode: sideways-lr;
    }

  span
    {
      display: block;
    }

  span.left-border
    {
      border-left: blue solid 1em;
    }

  span#right-border
    {
    border-right: blue solid 1em;
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
