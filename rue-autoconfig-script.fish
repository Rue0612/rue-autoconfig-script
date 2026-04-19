#!/usr/bin/env fish

set GITHUB_EMAIL '69476182+Rue0612@users.noreply.github.com'
set GITHUB_NAME 'Rue Coimbra'
set USER_NAME (whoami)
set MAX_SSH_ATTEMPTS 3
set CONFIG_FILE /etc/sddm.conf

printf "\n===> Initiating auto config...\n"

# Apps
# Util
# Neovim
printf "\n===> a) Installing necessary packages
(also autoinstalling apps)\n"
sudo pacman -Syu --noconfirm --needed \
	helium-browser-bin steam \
	git ttf-jetbrains-mono-nerd flatpak \
	neovim gcc make ripgrep fd tree-sitter-cli unzip wl-clipboard
printf "\n===> a) DONE\n"

printf "\n\n===> b) Installing flatpak packages and apps\n"
flatpak install -y flathub dev.vencord.Vesktop io.bassi.Amberol io.github.wivrn.wivrn
printf "\n===> b) DONE\n"

printf "\n\n ===> c) Configuring git\n"
read -P "Do you wanna configure Niri first? (In case keyboard not working) (Y\n): " answer
if test "$answer" != "n"
	printf "\nConfiguring Niri...\n"
	rm -rf ~/.config/niri
	git clone https://github.com/Rue0612/rue-niri-config.git ~/.config/niri
	printf "\n Niri configured!"
else
	printf "\nOk! Skiping Niri configuration!\n"
end
read -P "Do you want to skip the git configuration? (y\n): " answer
if test "$answer" != "y"
	git config --global user.name "$GITHUB_NAME"
	git config --global user.email "$GITHUB_EMAIL"

	printf "\n===> c) 1) Creating SSH keys\n"
	ssh-keygen -t ed25519 -C "$GITHUB_EMAIL"
	ssh-add ~/.ssh/id_ed25519
	printf "\n===> c) 1) DONE\n

	\n===> c) 2) Adding your SSH key to your GitHub Account"

	set attempt 1
	while test $attempt -le $MAX_SSH_ATTEMPTS

		wl-copy < ~/.ssh/id_ed25519.pub

		printf "\nYour SSH key is configured and copied to clipboard!"
		printf "\nPlease, go to https://github.com/settings/keys and add your key there!"
		printf "\nIf it is not in your clipboard, please run 'wl-copy <~/.ssh/id_ed25519.pub' to copy it again\n"
		read -P "Press enter when you are finished."

		printf "\n===> TESTING SSH CONNECTION\n"
		ssh -T git@github.com
		if test $status -eq 1
			break
		end

		printf "\n===> CONNECTION FAILED, trying again...\n"
		printf "\nERROR $status\n"
		set attempt (math $attempt + 1)
	end
	
	if test $attempt -gt $MAX_SSH_ATTEMPTS
		printf "\n===> c) 2) CONNECTION FAILED CATASTROFICALLY, skipping git config\n"
	else
		printf "\n===> c) 2) CONNECTED!\n"
	end
else
	printf "\n===> c) Skipping... \n"
end
printf "\n===> c) DONE\n"

printf "\n\n===> d) Configuring Niri Compositor\n"
rm -rf ~/.config/niri
git clone git@github.com:Rue0612/rue-niri-config.git ~/.config/niri
printf "\n===> d) DONE\n"
ssh-add ~/.ssh/id_ed25519
printf "\n\n===> e) Configuring Neovim\n"
rm -rf ~/.config/nvim/
git clone git@github.com:Rue0612/kickstart.nvim.git ~/.config/nvim/
printf "\n===> e) DONE\n"

printf "\n\n===> f) Configuring Alacritty\n"
rm -rf ~/.config/alacritty/
git clone git@github.com:Rue0612/rue-alacritty-config.git ~/.config/alacritty/
printf "\n===> f) DONE\n"

printf "\n\n===> g) Configuring Vesktop\n"
rm -rf /home/$USER_NAME/.var/app/dev.vencord.Vesktop/config/vesktop/themes
git clone git@github.com:Rue0612/rue-vesktop-theme.git /home/$USER_NAME/.var/app/dev.vencord.Vesktop/config/vesktop/themes
printf "\n===> g) DONE\n"ssh-add ~/.ssh/id_ed25519

printf "\n\n===> h) Configuring keymaps for lockscren\n"
printf "KEYMAP=colemak" | sudo tee /etc/vconsole.conf
sudo mkinitcpio -P
printf "\n===> h) DONE\n"

printf "\n\n===>i) Adding Autologin config\n"
if test -f $CONFIG_FILE
    if grep -q "\[Autologin\]" $CONFIG_FILE
        printf "Autologin section already exists, updating...\n"
        sudo sed -i "s/^User=.*/User=$USER_NAME/" $CONFIG_FILE
        sudo sed -i "s/^Session=.*/Session=niri/" $CONFIG_FILE
    else
        printf "Appending Autologin section...\n"
        printf "\n[Autologin]\nUser=$USER_NAME\nSession=niri" | sudo tee -a $CONFIG_FILE
    end
else
    printf "Creating $CONFIG_FILE...\n"
    printf "[Autologin]\nUser=$USER_NAME\nSession=niri" | sudo tee $CONFIG_FILE
end

printf "Done! Reboot to apply.\n"
printf "\n\n===>i) DONE"

printf "\n\n===j) Adding fish config\n"
rm -rf ~/.config/fish/
git clone git@github.com:Rue0612/rue-fish-config.git ~/.config/fish/
printf "\n===j) DONE!"

printf "\n\n===k) Adding fastfetch config\n"
rm -rf ~/.config/fastfetch/
mkdir ~/.config/fastfetch/
git clone git@github.com:Rue0612/rue-fastfetch-config.git ~/.config/fastfetch/
printf "\n===k) DONE!"

printf "\n\n\n===> Everything configured! Have fun!\n"
