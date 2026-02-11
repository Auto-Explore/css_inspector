# css/css-writing-modes/margin-collapse-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      font: 25px/1em Ahem;
      height: 4em;
    }

  div#wrapper
    {
      background: red url("support/margin-collapse-2em-space-wm-vert.png");
      width: 4em;
      writing-mode: vertical-rl;
    }

  div#wrapper > div
    {
      background-color: green;
      width: 1em;
    }

  div#rightmost
    {
      margin-left: 2em; /* block-end margin of 1st block */
    }

  div#leftmost
    {
      margin-right: 1em; /* block-start margin of following block */
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
