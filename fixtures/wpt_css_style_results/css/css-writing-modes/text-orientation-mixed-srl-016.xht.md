# css/css-writing-modes/text-orientation-mixed-srl-016.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/text-orientation-mixed-srl-016.xht"
}
```

## style[0]

```css
<![CDATA[
  @font-face
    {
      font-family: "mplus-1p-regular";
      src: url("/fonts/mplus-1p-regular.woff") format("woff");
      /* filesize: 803300 bytes (784.5 KBytes) */
    }

  div
    {
      border: gray solid 1px;
      font: bold 26px/32px "mplus-1p-regular";
      margin-bottom: 4px;
    }

  div#test-srl
    {
      writing-mode: sideways-rl;
      text-orientation: mixed;
    }

  div#ref-vrl-sideways
    {
      text-orientation: sideways;
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
