# css/css-writing-modes/margin-collapse-vrl-034.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/margin-collapse-vrl-034.xht"
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
      margin-right: 2em; /* block-start margin */
    }

  div#first-child
    {
      background-color: green;
      margin-right: 2em; /* block-start margin of child */
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
