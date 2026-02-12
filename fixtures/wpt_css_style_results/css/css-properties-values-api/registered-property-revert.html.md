# css/css-properties-values-api/registered-property-revert.html

```json
{
  "format_version": 3,
  "file": "css/css-properties-values-api/registered-property-revert.html"
}
```

## style[0]

```css

    #parent {
        --inherited: 10px;
        --non-inherited: 10px;
    }
    #child {
        --inherited: 20px;
        --non-inherited: 20px;
        --inherited: revert;
        --non-inherited: revert;
    }

    @keyframes revert_animation {
        from {
            --animated-inherited: revert;
            --animated-non-inherited: revert;
        }
        to {
            --animated-inherited: 100px;
            --animated-non-inherited: 100px;
        }
    }

    #animated_parent {
        --animated-inherited: 0px;
    }
    #animated_child {
        animation: revert_animation 10s -5s linear paused;
    }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
