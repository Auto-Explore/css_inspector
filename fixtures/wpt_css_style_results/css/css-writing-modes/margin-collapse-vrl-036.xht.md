# css/css-writing-modes/margin-collapse-vrl-036.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vrl-036.xht"
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
      border-right: green solid 1em;
      width: 3em;
      writing-mode: vertical-rl;
    }

  div#parent
    {
      background-color: green;
      margin-left: 2em; /* block-end margin */
    }

  div#last-child
    {
      margin-left: 2em; /* block-end margin of last child */
    }

  div#following-parent
    {
      background-color: green;
      width: 1em;
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
