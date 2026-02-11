# css/css-writing-modes/box-offsets-rel-pos-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/box-offsets-rel-pos-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  img
    {
      position: relative;
    }

  img.moves-toward-left
    {
      right: 80px;
    }

  img.moves-toward-top
    {
      bottom: 80px;
    }

  img.moves-toward-right
    {
      left: 80px;
    }

  img.moves-toward-bottom
    {
      top: 80px;
    }

  /*
  In this testcase, 7 red 80px by 80px squares overlap
  another red 80px by 80px square (placed in the center of
  a 3 by 3 grid of squares) and then, at the end, one single
  green 80px by 80px square overlaps all 8 other red squares.
  */
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
