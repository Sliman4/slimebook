# Transfer accounts (change account keys)

> WARNING: This is a dangerous operation. If you so something wrong, you won't be able to
> recover access to your account. Make sure you have a clear understanding of how [keys](main.md)
> work before doing this, and avoid doing this unless absolutely necessary.

The process is actually simpler than it sounds. You just need to create a new full access
key, and then delete the old one. Here's how you can do it in [Meteor Wallet](../../../lvl1/wallets/meteor-wallet.md):

1. Open the wallet:
   ![Open the wallet](transfer-account-1.png)
2. Click "Security and Recovery"
   ![Security and Recovery](transfer-account-2.png)
3. Click "Manage Full Access Keys"
   ![Manage Full Access Keys](transfer-account-3.png)
4. Remember your existing public key, you can copy it somewhere, or just remember the
   first 3 characters
5. Click "Add New Access Key"
   ![Add New Access Key](transfer-account-4.png)
6. Click "Generate New Key" (or if you want to transfer to a hardware device, choose
   this option)
   ![Generate New Key](transfer-account-5.png)
7. Save the seed phrase, and click "Continue"
8. When the key is added, the "Manage Full Access Keys" page will look like this:
   ![Manage Full Access Keys](transfer-account-6.png)
9. Either
    - Click "Set as Primary Key" near the new key so that the wallet now uses the new
      key to sign transactions. The new key is **not** the one that you remembered in
      step 4
    - Logout from your account and login with the new phrase
10. Now you can delete the old key by clicking "Revoke Access" next to it. The old key
    is the one that you remembered in step 4
    ![Revoke Access](transfer-account-7.png)

Now try to login with your old phrase/key, and you should see that it doesn't work
anymore. If you want to be sure, you can check the account's keys on the [explorer](../../../lvl3/nearblocks.md)
or in the wallet.

This method works in all wallets that offer full key management, but the interface might
look different.

## Why use it?

There are a few reasons why you might want to change your account keys:

- You lost your seed phrase, but you still have access to the wallet, and you want to
  transfer your account to a new one.
- You think that your old key might be compromised, and you want to create a new one.
  If you've leaked your seed phrase, or if you've used it on a compromised device, you
  should definitely change your keys.
