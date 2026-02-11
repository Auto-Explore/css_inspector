# css/css-syntax/escaped-eof.html

```json
{
  "format_version": 3,
  "file": "css/css-syntax/escaped-eof.html"
}
```

## style[0]

```css
foo { --foo:foo\
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Invalid escape at end of input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css
foo { --foo:1foo\
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Invalid escape at end of input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css
foo { --foo:url(foo\
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Invalid escape at end of input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css
foo { --foo:"foo\
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    },
    {
      "message": "Invalid escape at end of input.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
