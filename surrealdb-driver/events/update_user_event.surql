DEFINE EVENT update_user_accounts ON TABLE account WHEN $event = "CREATE" THEN (
    update user set accounts +=$value.id where id = $value.user_id
);


