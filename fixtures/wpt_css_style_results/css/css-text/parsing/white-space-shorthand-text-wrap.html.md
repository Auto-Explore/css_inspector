# css/css-text/parsing/white-space-shorthand-text-wrap.html

```json
{
  "format_version": 3,
  "file": "css/css-text/parsing/white-space-shorthand-text-wrap.html"
}
```

## style[0]

```css

.balance {
  text-wrap: balance;
}
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

#text-wrap-after-white-space {
  white-space: normal;
  text-wrap: balance;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

#white-space-after-text-wrap-balance {
  text-wrap: balance;
  white-space: normal;
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

#white-space-after-text-wrap-nowrap {
  text-wrap: nowrap balance;
  white-space: normal;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “text-wrap”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[4]

```css

.normal {
  white-space: normal;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
