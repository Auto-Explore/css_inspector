# css/css-writing-modes/inline-block-alignment-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/inline-block-alignment-003.xht"
}
```

## style[0]

```css
<![CDATA[
    div#lr-mixed
    {
      color: orange;
      font: 60px/1 Ahem; /* computes to 60px/60px */
      writing-mode: vertical-lr;
      text-orientation: mixed;
    }

    div#inline-block
    {
      display: inline-block;
      padding-left: 0.5em; /* computes to 60px */
      font-size: 2em; /* computes to 120px */
      /*
        such padding-left declaration is arbitrary and only serve to make the
        test a bit more challenging.
      */
    }

    span.block-descendant
    {
      display: block;
    }

    span#first-line-box
    {
      color: yellow;
    }

    span#last-line-box
    {
      color: blue;
    }

    span#orange30
    {
      padding-right: 4em; /* computes to 120px */
      font-size: 0.5em; /* computes to 30px */
      /*
        such padding-right declaration is arbitrary and only serve to make the
        test a bit more challenging.
      */
    }
    ]]>
```

```json
{
  "errors": 3,
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
    }
  ],
  "warnings": 0
}
```
