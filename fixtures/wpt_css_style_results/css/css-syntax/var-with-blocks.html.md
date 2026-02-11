# css/css-syntax/var-with-blocks.html

```json
{
  "format_version": 3,
  "file": "css/css-syntax/var-with-blocks.html"
}
```

## style[0]

```css

  .a {
    color: rgb(2, 2, 2);
    color:var(--x); /* Valid */
    background-color:rgb(1, 1, 1);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

  .a {
    color: rgb(2, 2, 2);
    color:{var(--x)}; /* Valid */
    background-color:rgb(1, 1, 1);
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

## style[2]

```css

  .a {
    color: rgb(2, 2, 2);
    color: { var(--x) }; /* Valid */
    background-color:rgb(1, 1, 1);
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

## style[3]

```css

  .a {
    color: rgb(2, 2, 2);
    color:var(--x) { }; /* Invalid */
    background-color:rgb(1, 1, 1);
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[4]

```css

  .a {
    color: rgb(2, 2, 2);
    color:{ } var(--x); /* Invalid */
    background-color:rgb(1, 1, 1);
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

## style[5]

```css

  .a {
    color: rgb(2, 2, 2);
    color:{ var(--x) } A; /* Invalid */
    background-color:rgb(1, 1, 1);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[6]

```css

  .a {
    color: rgb(2, 2, 2);
    color:A { var(--x) }; /* Invalid */
    background-color:rgb(1, 1, 1);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[7]

```css

  .a {
    --y:var(--x); /* Valid */
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[8]

```css

  .a {
    --y:{var(--x)}; /* Valid */
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

## style[9]

```css

  .a {
    --y: { var(--x) }; /* Valid */
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

## style[10]

```css

  .a {
    --y:var(--x) { }; /* Valid */
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[11]

```css

  .a {
    --y:{ } var(--x); /* Valid */
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

## style[12]

```css

  .a {
    --y:{ var(--x) } A; /* Valid */
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[13]

```css

  .a {
    --y:A { var(--x) }; /* Valid */
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
