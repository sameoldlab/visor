<script>
    export let title = ""
    export let data
</script>

<div>
    <span class="title">{title}</span>
    <span class="data">{data}</span>
</div>

<style lang="scss">
    div {
        display: flex;
        flex-direction: column;
        gap: 4px;

        padding: 10px 20px;
        // border: 1px solid var(--border);
    }
    .title {
        line-height: 21px;
        font-weight: 500;
        text-transform: uppercase;
        color: #BABABA;
    }
    .data {
        font-family: var(--mono);
    }
</style>