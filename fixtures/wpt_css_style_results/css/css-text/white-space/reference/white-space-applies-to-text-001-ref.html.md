# css/css-text/white-space/reference/white-space-applies-to-text-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-text/white-space/reference/white-space-applies-to-text-001-ref.html"
}
```

## style[0]

```css

  section
    {
      border: black solid 2px;
      float: left;
      font-family: monospace;
      font-size: 14px;
      line-height: 1; /* computes to 14px */
      margin-bottom: 10px;
      width: 16ch;
    }

  section.odds
    {
      clear: both;
    }

  section.even
    {
      margin-left: 10em;
    }

  div.first-subtest
    {
      white-space: normal;
    }

  div.second-subtest
    {
      white-space: nowrap;
    }

  div.third-subtest
    {
      white-space: pre;
    }

  div.fourth-subtest
    {
      white-space: pre-wrap;
    }

  div.fifth-subtest
    {
      white-space: break-spaces;
    }

  div.sixth-subtest
    {
      white-space: pre-line;
    }

  hr
    {
      clear: both;
      margin: 4em auto 1.5em 0em;
      width: 480px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
