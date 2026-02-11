# css/css-mixins/mixin-layers.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-layers.html"
}
```

## style[0]

```css

    @layer one, two;
  
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

    @layer one {
      @mixin --m1() {
        @result {
          color: green;
        }
      }
    }

    #e1 {
      @apply --m1();
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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

## style[2]

```css

    @layer {
      @mixin --m2() {
        @result {
          color: green;
        }
      }
    }

    #e2 {
      @apply --m2();
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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

## style[3]

```css

    @layer two {
      @mixin --m3() {
        @result {
          color: green;
        }
      }
    }

    @layer one {
      @mixin --m3() {
        @result {
          color: red;
        }
      }
    }

    #e3 {
      @apply --m3();
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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

## style[4]

```css

    @layer one {
      @mixin --m4() {
        @result {
          color: red;
        }
      }
    }

    @layer two {
      @mixin --m4() {
        @result {
          color: green;
        }
      }
    }

    #e4 {
      @apply --m4();
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
