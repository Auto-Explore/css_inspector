# css/css-writing-modes/sizing-orthog-prct-vrl-in-htb-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-prct-vrl-in-htb-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      padding: 100px 0px;
      writing-mode: vertical-rl;
    }

  body
    {
      font-size: 16px;
      line-height: 1.25; /* therefore, each line box is 20px tall */
    }

  p
    {
      left: 8px;
      position: absolute;
      writing-mode: horizontal-tb;
    }

  p#sentence-before
    {
      margin-top: 0px;
      top: 100px;
    }

  div
    {
      border: blue solid 35px;
      height: 50%;
      left: 8px;
      position: absolute;
      top: 136px;
    }

  p#sentence-after
    {
      top: calc(100px - 16px + 52px + 35px + 50vh + 35px);
      /*
      50vh means 50 vh unit where each vh is equal to
      1% of the height of the initial containing block.
      So 50vh == half of the height of initial containing block
      5.1.2. Viewport-percentage lengths: the vw, vh, vmin, vmax units
      https://www.w3.org/TR/css3-values/#vh
      */
      padding-bottom: 100px;
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
