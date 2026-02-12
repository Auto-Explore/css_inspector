# css/css-writing-modes/sizing-orthog-vrl-in-htb-020-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/sizing-orthog-vrl-in-htb-020-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  body
    {
      font-size: 16px;
      font-family: monospace;
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
      margin-top: 8px;
    }

  div
    {
      border: blue solid 3px;
      height: auto;
      left: 8px;
      position: absolute;
      top: 52px;
    }

  p#sentence-after
    {
      top: calc(52px + 3px + 50ch + 3px);
      /*
      50ch means 50 ch unit where each ch is equal to
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
