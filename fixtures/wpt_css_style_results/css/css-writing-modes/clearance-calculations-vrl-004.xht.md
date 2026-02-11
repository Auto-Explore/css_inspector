# css/css-writing-modes/clearance-calculations-vrl-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/clearance-calculations-vrl-004.xht"
}
```

## style[0]

```css
<![CDATA[
  div
    {
      height: 5em; /* computes to 100px */
    }

  div#parent
    {
      background: red url("support/clearance-calculation-vrl-004.png");
      font: 20px/1 Ahem; /* computes to 20px/20px */
      min-width: 8em;
      writing-mode: vertical-rl;
    }

  div > div
    {
      background-color: green;
      width: 1em;
    }

  div#preceding-sibling
    {
      margin-left: 4em;
    }

  div#floated-left
    {
      float: left;
      width: 2em;
    }

  div#clearing-left
    {
      clear: left;
      margin-right: 3em;
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
