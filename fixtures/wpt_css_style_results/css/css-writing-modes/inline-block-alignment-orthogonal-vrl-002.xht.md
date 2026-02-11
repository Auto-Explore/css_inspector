# css/css-writing-modes/inline-block-alignment-orthogonal-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/inline-block-alignment-orthogonal-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
    div#horiz-tb
    {
      color: orange;
      font: 60px/1 Ahem; /* computes to 60px/60px */
      writing-mode: horizontal-tb;
    }

    div#inline-block-90
    {
      background-color: orange; /* we want the padding-bottom to be painted with orange color */
      display: inline-block;
      font-size: 1.5em; /* computes to 90px */
      /*
        such padding-bottom declaration is arbitrary and only serve to make the
        test a bit more challenging.
      */
      padding-bottom: 0.5em; /* computes to 45px */
      text-orientation: mixed;
      writing-mode: vertical-rl;
    }

    span.block-descendant
    {
      display: block;
    }

    span#orange30
    {
      font-size: 0.5em; /* computes to 30px */
    }
    ]]>
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
