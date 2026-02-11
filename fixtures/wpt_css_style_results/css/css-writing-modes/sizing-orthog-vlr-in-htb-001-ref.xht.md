# css/css-writing-modes/sizing-orthog-vlr-in-htb-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vlr-in-htb-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      height: 100%;
      padding: 100px 0px;
      writing-mode: vertical-lr;
    }

  body
    {
      font-size: 16px;
      height: 100%;
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
      border: blue solid 3px;
      box-sizing: border-box;
      height: 100%;
      left: 8px;
      position: absolute;
      top: 136px;
    }

  p#sentence-after
    {
      padding-bottom: 100px;
      top: calc(100px - 16px + 52px + 100vh);
      /*
      100vh means 100 vh unit where each vh is equal to
      1% of the height of the initial containing block.
      So 100vh == height of initial containing block
      5.1.2. Viewport-percentage lengths: the vw, vh, vmin, vmax units
      https://www.w3.org/TR/css3-values/#vh
      */
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
