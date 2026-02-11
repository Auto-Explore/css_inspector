# css/css-syntax/custom-property-rule-ambiguity.html

```json
{
  "format_version": 3,
  "file": "css/css-syntax/custom-property-rule-ambiguity.html"
}
```

## style[0]

```css

  .a { }
  --x:hover { } /* Ambiguous "rule" */
  .b { }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  .a { }
  --x:hover { ] } /* Ambiguous "rule" */
  .b { }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

  div {
    .a { }
    --x:hover { }
    .b { }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

  div {
    .a { }
    --x:hover { ] }
    .b { }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
