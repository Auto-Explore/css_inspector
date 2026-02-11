# css/css-writing-modes/sizing-orthog-vrl-in-htb-003-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vrl-in-htb-003-ref.xht"
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
      border: blue solid 3px;
      height: auto;
      left: 8px;
      position: absolute;
      top: 136px;
    }

  p#sentence-after
    {
      padding-bottom: 100px;
      top: calc(136px + 3px + 15ch + 3px);
      /*
      15ch means 15 ch unit where each ch is equal to
      the used advance measure of the "0" (ZERO, U+0030) glyph found
      in the font used to render it.
      5.1.1. Font-relative lengths: the em, ex, ch, rem units
      https://www.w3.org/TR/css3-values/#ch
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
