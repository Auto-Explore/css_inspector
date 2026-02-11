# css/css-writing-modes/underline-font-size-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/underline-font-size-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  div#vrl
    {
      font-size: 4em;
      margin-left: 2em;
      writing-mode: vertical-rl;
    }

  span#parent
    {
      text-decoration: underline;
    }

  span#child
    {
      color: transparent;
    }

  span#larger
    {
      font-size: 1.5em;
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
